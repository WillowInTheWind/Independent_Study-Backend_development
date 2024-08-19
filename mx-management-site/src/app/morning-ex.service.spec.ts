import { TestBed } from '@angular/core/testing';

import { MorningExService } from './morning-ex.service';

describe('MorningExService', () => {
  let service: MorningExService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MorningExService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
